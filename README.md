## My First Desktop Application Built with Rust and Slint
After dedicating the past four months to studying the Rust programming language seriously, I found it challenging but remained determined to complete the course. I read the [Rust Language Book](https://doc.rust-lang.org/stable/book/title-page.html) and watched YouTube tutorials:[Learn Rust Programming - Complete Course](https://youtu.be/BpPEoZW5IiY) for a total of 14 hours to study and review. However, I typically spent no more than 30 minutes per day—or even less—because I needed time to practice the exercises provided by the instructor.

It took me about a month to finish the YouTube course. One particular tutorial stood out—it was exceptionally well-taught, paced slowly, and never felt rushed. The explanations were so detailed that anything unclear from the book became much easier to understand after watching the video.

Reading books provides the theoretical foundation, but applying that knowledge is where the real integration happens. It tests whether the understanding gained is sufficient to write a functional program. Can that knowledge piece together the jigsaw puzzle of solving real-world problems?

I use Slint for graphical user interfaces. The Slint project was first introduced in early 2022, making it a relatively new framework. Despite its young age, it has gained attention for its modern design, Rust-based architecture, and focus on lightweight, efficient UI creation. Its growth and adoption indicate a promising future as developers explore alternatives to more established frameworks like Qt.

From my initial experience, Slint is still not as feature-rich as Qt. However, learning and using Slint to create a user interface is relatively straightforward. With some study and practice, you can quickly grasp its concepts and effectively design UI elements.

First, clone my repository. Then, use Cargo to build and run the code.

```console
PS C:\Users\priab\projects>git clone https://github.com/pbroboto/geoecef
PS C:\Users\priab\projects>cd geoecef
PS C:\Users\priab\projects\geoecef>cargo build
PS C:\Users\priab\projects\geoecef>cargo run
```
You will my first desktop application built with Rust and Slint:
The calculation converts coordinates from latitude and longitude (Geographic) to Cartesian (ECEF) and vice versa.

![GeoEcef on first run](https://github.com/pbroboto/geoecef/blob/main/images/geoecef_first_run.jpg)

Click the left or right bulb for examples, and use the arrow to set the direction of the calculation. Click the calculator icon to transform the coordinates. To clear the data, click the eraser icon, and to copy the result to the clipboard, click the copy icon.

![GeoEcef](https://github.com/pbroboto/geoecef/blob/main/images/geoecef_feature_image.png)
