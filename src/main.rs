// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//use latlon::GeoParseError;
use arboard::Clipboard;
use latlon::GeoParseError;
use slint::Color;
use std::{num::ParseFloatError, rc::Rc, sync::Mutex};

// WGS84 constants
const A: f64 = 6378137.0; // Semi-major axis (meters)
const F: f64 = 1.0 / 298.257223563; // Flattening
const E2: f64 = 2.0 * F - F * F; // Square of eccentricity
const EPSILON: f64 = 1e-12; // Convergence tolerance

slint::include_modules!();
fn main() {
    let app = MainWindow::new().unwrap();
    let clipboard = Rc::new(Mutex::new(Clipboard::new().unwrap()));
    let cb1 = clipboard.clone();
    let cb2 = clipboard.clone();

    app.on_erase_latlon({
        let app_weak = app.as_weak();
        let ui = app_weak.unwrap();
        move || {
            ui.set_xlon("".into());
            ui.set_ylat("".into());
            ui.set_zhi("".into());
        }
    });

    app.on_erase_ecef({
        let app_weak = app.as_weak();
        let ui = app_weak.unwrap();
        move || {
            ui.set_xcart("".into());
            ui.set_ycart("".into());
            ui.set_zcart("".into());
        }
    });

    app.on_example_latlon({
        let app_weak = app.as_weak();
        let ui = app_weak.unwrap();
        move || {
            ui.set_ylat("15°13'12.1252\"N".into());
            ui.set_xlon("100 12 12.3256 E".into());
            ui.set_zhi("5.202".into());
        }
    });

    app.on_example_ecef({
        let app_weak = app.as_weak();
        let ui = app_weak.unwrap();
        move || {
            ui.set_xcart("-1070053.3249".into());
            ui.set_ycart("6068573.9676".into());
            ui.set_zcart("1640100.7872".into());
        }
    });

    app.on_copy_latlon({
        let app_weak = app.as_weak();
        let ui = app_weak.unwrap();
        move || {
            let y = ui.get_ylat();
            let x = ui.get_xlon();
            let z = ui.get_zhi();
            let the_string = [y, x, z].join(",");
            cb1.lock().unwrap().set_text(the_string).unwrap();
        }
    });

    app.on_copy_ecef({
        let app_weak = app.as_weak();
        let ui = app_weak.unwrap();
        move || {
            let x = ui.get_xcart();
            let y = ui.get_ycart();
            let z = ui.get_zcart();
            let the_string = [x, y, z].join(",");
            cb2.lock().unwrap().set_text(the_string).unwrap();
        }
    });

    app.on_transform_coordinate({
        let app_weak = app.as_weak();
        let ui = app_weak.unwrap();
        move |calc_dir: CalculateDirection, in_coor3d: Coordinate3D| {
            let (xx, yy, zz): (f64, f64, f64);
            if calc_dir == CalculateDirection::Ecef2Geo {
                //check validity of input values.
                (xx, yy, zz) = match parse_ecef(&in_coor3d) {
                    Ok(val) => {
                        ui.set_status("".into());
                        (val.0, val.1, val.2)
                    }
                    Err(e) => {
                        ui.set_status_color(Color::from_rgb_u8(255, 00, 0));
                        ui.set_status(format!("Error: {}", e).into());
                        return ();
                    }
                };
            } else {
                (xx, yy) = match parse_latlong(&in_coor3d) {
                    Ok(val) => {
                        ui.set_status("".into());
                        (val.0, val.1)
                    }
                    Err(e) => {
                        ui.set_status_color(Color::from_rgb_u8(255, 00, 0));
                        ui.set_status(format!("{}", e).into());
                        return ();
                    }
                };
                zz = match in_coor3d.z.parse::<f64>() {
                    Err(e) => {
                        ui.set_status_color(Color::from_rgb_u8(255, 00, 0));
                        ui.set_status(format!("Error: {}", e).into());
                        return ();
                    }
                    Ok(val) => {
                        ui.set_status("".into());
                        val
                    }
                };
            }
            println!("parsed latlon xx, yy, zz: {}, {}, {}", xx, yy, zz);
            // Calculate the results
            if calc_dir == CalculateDirection::Geo2Ecef {
                let (x, y, z) = geodetic_to_ecef(yy, xx, zz);
                println!("output x, y, z: {}, {}, {}", x, y, z);
                // Output Ecef coordinates
                ui.set_xcart(format!("{:.4}", x).into());
                ui.set_ycart(format!("{:.4}", y).into());
                ui.set_zcart(format!("{:.4}", z).into());
            } else {
                let (y, x, z) = ecef_to_geodetic(xx, yy, zz);
                println!("output x, y, z: {}, {}, {}", x, y, z);
                // Output geographic coordinates
                ui.set_xlon(format!("{:.8}", x).into());
                ui.set_ylat(format!("{:.8}", y).into());
                ui.set_zhi(format!("{:.4}", z).into());
            }
        }
    });

    app.run().unwrap();
}

fn parse_latlong(coor3d: &Coordinate3D) -> Result<(f64, f64), GeoParseError<&slint::SharedString>> {
    let yy = match latlon::parse_lat(&coor3d.y) {
        Err(e) => return Err(e),
        Ok(val) => val,
    };
    let xx = match latlon::parse_lng(&coor3d.x) {
        Err(e) => return Err(e),
        Ok(val) => val,
    };
    Ok((xx, yy))
}

fn parse_ecef(coor3d: &Coordinate3D) -> Result<(f64, f64, f64), ParseFloatError> {
    //check validity of input values.
    let yy = match coor3d.y.parse::<f64>() {
        Err(e) => {
            return Err(e);
        }
        Ok(val) => val,
    };
    let xx = match coor3d.x.parse::<f64>() {
        Err(e) => {
            return Err(e);
        }
        Ok(val) => val,
    };
    let zz = match coor3d.z.parse::<f64>() {
        Err(e) => {
            return Err(e);
        }
        Ok(val) => {
            val
        }
    };
    Ok((xx, yy, zz))
}

/// Converts Cartesian (ECEF) coordinates to latitude, longitude, and height.
fn ecef_to_geodetic(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let lon = y.atan2(x).to_degrees(); // Longitude in degrees

    let p = (x.powi(2) + y.powi(2)).sqrt(); // Distance from z-axis
    let mut lat = (z / p).atan(); // Initial guess for latitude

    // Iterative computation for latitude
    loop {
        let sin_lat = lat.sin();
        let n = A / (1.0 - E2 * sin_lat.powi(2)).sqrt();
        let new_lat = (z + E2 * n * sin_lat) / p;
        let new_lat = new_lat.atan();

        if (new_lat - lat).abs() < EPSILON {
            lat = new_lat;
            break;
        }
        lat = new_lat;
    }

    // Final calculation for height
    let sin_lat = lat.sin();
    let n = A / (1.0 - E2 * sin_lat.powi(2)).sqrt();
    let height = p / lat.cos() - n;

    (lat.to_degrees(), lon, height) // Latitude and longitude in degrees, height in meters
}

/// Converts latitude, longitude, and height to Cartesian (ECEF) coordinates.
fn geodetic_to_ecef(lat: f64, lon: f64, height: f64) -> (f64, f64, f64) {
    //println!("Test lat, lon, height: {}, {}, {}", lat, lon, height);
    // Convert latitude and longitude from degrees to radians
    let lat_rad = lat.to_radians();
    let lon_rad = lon.to_radians();

    // Calculate the prime vertical radius of curvature
    let sin_lat = lat_rad.sin();
    let n = A / (1.0 - E2 * sin_lat * sin_lat).sqrt();

    // Compute Cartesian coordinates
    let x = (n + height) * lat_rad.cos() * lon_rad.cos();
    let y = (n + height) * lat_rad.cos() * lon_rad.sin();
    let z = (n * (1.0 - E2) + height) * sin_lat;

    (x, y, z)
}
