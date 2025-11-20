Ok so I want this app to consist of 2 main components.

# Final Project Requirements
## Camera App:
A mobile/desktop app built with tauri that will
- Take the camera app and record video, using a webcam on desktop and the builtin camera on mobile.
- Encode that video using two potential options. 
  - On desktop it will use rav1e to encode the stream using av1. (I have looked at some libraries but rav1e looks to be good for this.)
  - On mobile it will attempt to use hardware h265 encoding to compactify the stream. If that isnt availible it will use the same encoder as the desktop app.
- Send that video to the livestreaming server at a specific IP you can set in the app config.
- Enable some kind of control plane on the device, so that the livestraming/control server can tell the client when to start and stop the livestream as well as adjusting other settings.
(The desktop app is meant mainly for testing and to try and reduce complexity as much as possible for an MVP)
In particular bandwidth on this device is extremely limited. (It is in a hard to service location where we have to pay for any cellular bandwith we use. However the hardware requirements havent been fully speced out yet. We were just planning on using a 2022 android phone as the brains, but if encoding the livestream is more intense we can swap out hardware)

## Control and Livestreaming Server
A Server that would consume the livestream from the camera app. And do the following with it.
- Serve a small frontend that would allow you to watch said livestream. (This is for internal monitoring oftentimes using powerful desktops with gigabit internet are so so much less intense then the requirements sending the livestream to the server. For simplicity it should just copy and forward the stream without worrying about re-encoding anything, and exclude any unnecessary complexity like adaptive streaming.)


# MVP Requirements

For an absolutely minimal MVP I want to:
1. Only build the desktop app using tauri. Capture the desktop camera. Use rav1e for the av1 encoding. And whatever library you think would be appropriate for a websocket/webrtc client.

2. For the server just build something super simple that would take in a websocket stream and print something to indicate it is receiving the stream successfully. Axum (a web framework I have used previously) has support for websockets and some example code exists currently at /home/nicole/Documents/mycorrhizae/flumph/flumph-server/src/main.rs

(However after doing a bunch of research it seems like websockets are actually not a great way to handle streaming live video, and that its probably a good idea to just skip straight to using WebRTC with the cannonical webrtc library: https://github.com/webrtc-rs/webrtc)


What in your opinion would be the best way to proceed with this mvp architecture wise?

To keep things as simple as possible 

I want to do this in rust as much as possible. For the main architecture I want 

1. To be written using tauri/react. But the av1 encoding should be done using rust code and tauri. It does seem like tauri got the ability to make mobile apps in the last couple years and execute mobile code directly with plugins 

```
Mobile Plugin Development
Be sure that you’re familiar with the concepts covered in the Plugin Development guide as many concepts in this guide build on top of foundations covered there.
Plugins can run native mobile code written in Kotlin (or Java) and Swift. The default plugin template includes an Android library project using Kotlin and a Swift package including an example mobile command showing how to trigger its execution from Rust code.

Develop an Android Plugin

A Tauri plugin for Android is defined as a Kotlin class that extends app.tauri.plugin.Plugin and is annotated with app.tauri.annotation.TauriPlugin. Each method annotated with app.tauri.annotation.Command can be called by Rust or JavaScript.

Tauri uses Kotlin by default for the Android plugin implementation, but you can switch to Java if you prefer.


ExamplePlugin:
import android.app.Activity
import android.webkit.WebView
import app.tauri.annotation.TauriPlugin
import app.tauri.annotation.InvokeArg

@InvokeArg
class Config {
    var timeout: Int? = 3000
}

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
  private var timeout: Int? = 3000

  override fun load(webView: WebView) {
    getConfig(Config::class.java).let {
       this.timeout = it.timeout
    }
  }
}
```

2. The web server should be written using something like axum. I dont know how I could get it to serve video. Could you help me write out a description of this application?

Could you help me chat through the architecture and let me know what tools and libraries I should be using for this?
