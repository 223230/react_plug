<div align="center">
<h1>
<img src="https://github.com/user-attachments/assets/000ce122-5dc2-40b6-9a63-bf8942bc3b79">
<div>React-Plug</div>
</h1>
<p>
A new way to build plug-ins using Rust and React.
</p>

![Lines of Code](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fapi.codetabs.com%2Fv1%2Floc%2F%3Fgithub%3D223230%2Freact_plug%26branch%3Dmain&query=%24%5B%3F(%40.language%3D%3D%22Rust%22)%5D.linesOfCode&label=Lines%20of%20Code&labelColor=gray&color=blue)
[![Test](https://github.com/223230/react_plug/actions/workflows/test.yml/badge.svg)](https://github.com/223230/react_plug/actions/workflows/test.yml)

</div>

---

> [!CAUTION]
> **Turn away before it's too late!**
> 
> You've stumbled upon this project at an extremely early stage! Re-visit it
> when it's more mature. Right now, I'm just trying things out and the whole
> project might end up going in a totally different direction. That, or I'll
> just lose interest.

React-Plug is a crate that allows you to build Rust audio plug-ins with React
GUIs using [nih-plug](https://github.com/robbert-vdh/nih-plug) and
[nih-plug-webview](https://github.com/httnn/nih-plug-webview). It bundles and
includes your React GUI, automatically generates
[TypeScript](https://typescriptlang.org) bindings for it, handles Plugin/GUI
communication and more. It's meant to be a batteries-included easy-to-use
framework. Here are some of its standout features:

  - [x] Macros for easy parameter generation
  - [x] Automatically generated TypeScript bindings
  - [ ] Support for custom messages
  - [ ] Hot-reloading (dev mode)

## Parameters

For ease of use, you will be defining your parameters via a procedural macro,
as this framework uses a DSL (domain specific language) for parameter
definitions. Here's an example:

```rust
rp_params! {
    ExampleParams {
        gain: FloatParam {
            name: "Gain",
            value: util::db_to_gain(0.0),
            range: FloatRange::Skewed {
                min: util::db_to_gain(-30.0),
                max: util::db_to_gain(30.0),
                factor: FloatRange::gain_skew_factor(-30.0, 30.0),
            },
            smoother: SmoothingStyle::Logarithmic(50.0),
            unit: " dB",
            value_to_string: formatters::v2s_f32_gain_to_db(2),
            string_to_value: formatters::s2v_f32_gain_to_db(),
        },
        muted: BoolParam {
            name: "Muted",
            value: false
        },
    }
}
```

One downside is that the `new` function of your parameters will be generated by
React-Plug and you can really only define your parameter defaults inside the
`rp_params!` macro. You also won't be able to add other fields to your params
struct. If any proc macro wizard/witch/wix wants to help me out with this, file
a pull request, create an issue, or cast a spell on my codebase. I'd love for
there to be a more flexible solution.
