# Control Box Simulator Extension

This project extends the simulator by adding support for **custom signals** and **custgom elements** using [Yew](https://yew.rs/), [Trunk](https://trunkrs.dev/), and [TailwindCSS](https://tailwindcss.com/).

This repository is treated as working example that shows how an custom signal and a custom element is added.

## Features

- **Signals**: Define and manage custom signals within the simulator.
- **Elements**: Add and configure new elements for simulation.
- **Modern UI**: Built with Yew for fast, reactive web interfaces.
- **Styling**: Uses TailwindCSS for utility-first styling.
- **Build Tooling**: Powered by Trunk for easy development and deployment.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Trunk](https://trunkrs.dev/#install)
- [Node.js](https://nodejs.org/) (for TailwindCSS)

### Setup

1. **Install dependencies:**
    ```bash
    cargo install trunk
    npm install
    ```

1. **Create tailwindcss output:**
    ```bash
    npm run build:css
    ```

2. **Run the development server:**
    ```bash
    trunk serve
    ```

3. **Build for production:**
    ```bash
    trunk build --release
    ```

## Project Structure

- `src/` - Yew components and Rust logic
- `tailwind.config.js` - TailwindCSS configuration

## Customization

- Add new signals in `src/time_signal`
- Add new elements in `src/plant`
- Update styles in `./index.css`

## License

MIT — see [`LICENSE.md`](LICENSE.md)
