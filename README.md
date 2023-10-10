<div id="top"></div>
<!--
*** This README was created with https://github.com/othneildrew/Best-README-Template
-->

<!-- PROJECT SHIELDS -->

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->

<br />
<div align="center">

<h3 align="center">Template Rust</h3>

<p align="center">
    This template provides just enough to get started with your Rust application.
    <br />
    <br />
    <a href="https://github.com/lenra-io/template-rust/issues">Report Bug</a>
    ·
    <a href="https://github.com/lenra-io/template-rust/issues">Request Feature</a>
  </p>
</div>

## List of managed features

- [ ] Lenra views
  - [ ] find

    - [X] $eq
    - [ ] $ne
    - [ ] $gt
    - [ ] $gte
    - [ ] $lt
    - [ ] $lte
    - [ ] $in
    - [ ] $nin
    - [ ] $and
    - [ ] $or
    - [ ] $not
    - [ ] $nor
    - [ ] $exists
    - [ ] $type
    - [ ] $all
  - [X] props
  - [ ] context
- [ ] Lenra components (with every listeners)
  - [ ] Actionable
  - [ ] Button
  - [ ] Carousel
  - [ ] Checkbox
  - [ ] Container
  - [ ] DropDownButton
  - [ ] Flex
  - [ ] Flexible
  - [ ] Form
  - [ ] Icon
  - [ ] Image
  - [ ] Menu
  - [ ] MenuItem
  - [ ] OverlayEntry
  - [ ] Radio
  - [ ] Slider
  - [ ] Stack
  - [ ] StatusSticker
  - [ ] Text
  - [ ] TextField
  - [ ] Toggle
  - [X] View
  - [ ] Wrap
- [ ] JSON views
  - [ ] listeners (with custom event values)
- [ ] Lenra listeners
  - [ ] System listeners

    - [ ] onEnvStart
    - [ ] onUserFirstJoin
    - [ ] onSessionStart

    <!-- - [ ] onSessionStop
    - [ ] onUserLeave
    - [ ] onEnvStop -->
- [ ] Query
  - [ ] Find

    - [ ] operators
    - [ ] projections

    <!-- - [ ] options -->
  - [ ] UpdateMany
  - [ ] CreateDoc
  - [ ] UpdateDoc
  - [ ] DeleteDoc
  - [ ] GetDoc
  - [ ] Transactions
- [ ] Cron
- [ ] @lenra:navTo
  - [X] app routes
  - [ ] external URL
- [ ] Webhooks

<!-- GETTING STARTED -->

## Prerequisites

To properly run this template, you will have to make sure that the Lenra CLI and docker with docker-compose are installed on your computer.
Installation instructions can be found here https://github.com/lenra-io/lenra_cli.

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->

## Usage

To create a new app from this template use `lenra new` command:

```console
lenra new rust --path my_rust_app
```

Then you can build and start it with the `lenra dev` command:

```console
lenra dev
```

You can then access the application by opening [`localhost:4000`](http://localhost:4000) on your web browser.

This template is a basic implementation of a Lenra application using the Rust language. You can get your application started by using this template.

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please open an issue with the tag "enhancement".
Don't forget to give the project a star if you liked it! Thanks again!

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- LICENSE -->

## License

Distributed under the **MIT** License. See [LICENSE](./LICENSE) for more information.

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

Lenra - [@lenra_dev](https://twitter.com/lenra_dev) - contact@lenra.io

Project Link: [https://github.com/lenra-io/template-rust](https://github.com/lenra-io/template-rust)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->

<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/lenra-io/template-rust.svg?style=for-the-badge
[contributors-url]: https://github.com/lenra-io/template-rust/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/lenra-io/template-rust.svg?style=for-the-badge
[forks-url]: https://github.com/lenra-io/template-rust/network/members
[stars-shield]: https://img.shields.io/github/stars/lenra-io/template-rust.svg?style=for-the-badge
[stars-url]: https://github.com/lenra-io/template-rust/stargazers
[issues-shield]: https://img.shields.io/github/issues/lenra-io/template-rust.svg?style=for-the-badge
[issues-url]: https://github.com/lenra-io/template-rust/issues
[license-shield]: https://img.shields.io/github/license/lenra-io/template-rust.svg?style=for-the-badge
[license-url]: https://github.com/lenra-io/template-rust/blob/master/LICENSE
