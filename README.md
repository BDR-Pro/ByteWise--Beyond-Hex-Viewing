# ByteWise: Beyond Hex Viewing

Hey there! 👋 Welcome to the Super Cool Hex Editor - the quirky little tool that makes digging into files not only insightful but also a bit of fun. This nifty app lets you peek into the soul of your executables and other mysterious files in a way that's a little less "Matrix" and a bit more "Magic School Bus".

## What's It Do?

In a nutshell, our app lets you:

- 🎨 View file contents in hex and get a color representation based on the byte values - because who said hex viewing had to be boring?
- 🔍 Extract and display details from executable files because sometimes you gotta know what's under the hood.
- 🔄 Toggle between different views (Hex, Color, and Preferences) because variety is the spice of life.

## How It Works

Under the hood (don't worry, it's not scary), we use `druid` for the GUI magic and `reverse_engineering_lib` for the heavy lifting. The code is a beautiful symphony of Rust's safety and efficiency, so you can run it on your machine without summoning any ancient spirits (unless you want to).

### Key Features

- **Hex View**: A straightforward view where you can see the contents of your file in hex. Perfect for the traditionalists out there.
- **Color View**: Adds a bit of pizzazz by displaying your file's bytes as colors. It's like a party for your eyes!
- **Executable Details**: Digs into executable files to show you what's going on behind those mysterious binary curtains.

### Getting Started

1. Clone this repository. If you're not sure how, a quick web search on "how to clone a GitHub repository" will set you on the right path.
2. Ensure you have Rust installed. Again, the internet has plenty of guides on this.
3. Build and run the project with `cargo run`. If this sounds like magic, check out Rust's documentation for more details.

### Using the App

- Launch the app, and you'll be greeted with a friendly UI.
- Click "Toggle View" to switch between Hex, Color, and Preferences views. It's like channel surfing but for data!
- Open a file, and watch the magic happen. If nothing happens, make sure you selected a file (it's easy to miss).

## Contributing

Got an idea? A suggestion? Found a bug that's bugging you? Jump into the issues section and let us know. Or better yet, fork this project, make your improvements, and hit us with a pull request. All contributions are welcome!

## License

It's all open source! Check out the LICENSE file for the nitty-gritty legal stuff.

---

Hope you enjoy using this as much as we enjoyed making it. Happy hexing!
