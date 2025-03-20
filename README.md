# imgdup

imgdup is an application that searches for similar images based on registered images.

## Development

### Install the following tools

- Rust
  - <https://www.rust-lang.org/tools/install>
- Dioxus cli
  - <https://dioxuslabs.com/learn/0.6/getting_started/#install-the-dioxus-cli>
- tailwind cli
  - <https://tailwindcss.com/docs/installation/tailwind-cli>
- Task
  - <https://taskfile.dev>

### Setup

Clone the repository:

```bash
git clone https://github.com/Takashicc/imgdup
```

Install the npm dependencies:

```bash
npm i
```

Run the following command to create the [./db.sqlite](/db.sqlite) file:

```bash
task migration:up
```

Run the following command to generate the [./assets/tailwind.css](/assets/tailwind.css) file:

```bash
task tailwind
```

Open a new terminal window and run the following command to build, watch & serve the Dioxus project:

```bash
dx serve
```
