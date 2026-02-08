# Connect 4 Frontend

This is the frontend UI for the Connect 4 Agent project, built with Vue 3, TypeScript, and Vite. It provides an interactive game board where users can play Connect 4 against an AI agent powered by a Rust backend.

## Features

- **Interactive Game Board**: Click on columns to drop pieces and play against the AI
- **Visual Feedback**: 
  - Highlighting of winning tiles
  - Display of the last played move
  - Preview of potential winning moves
- **Game State Management**: Automatic detection of wins and draws
- **Responsive Design**: Clean, modern UI with smooth animations
- **Backend Integration**: Communicates with the Rust backend API to get optimal AI moves

## Prerequisites

- Node.js (v18 or higher recommended)
- npm or yarn package manager
- Backend server running on `http://localhost:8081` (see main README for backend setup)

## Project Setup

Install dependencies:

```sh
npm install
```

### Development

Run the development server with hot-reload:

```sh
npm run dev
```

The frontend will be available at `http://localhost:5173` (or the port shown in the terminal).

**Note**: Make sure the backend server is running on port 8081 before playing the game.

### Production Build

Type-check, compile and minify for production:

```sh
npm run build
```

Preview the production build:

```sh
npm run preview
```

### Code Quality

Lint and fix code with ESLint:

```sh
npm run lint
```

Format code with Prettier:

```sh
npm run format
```

Type-check without building:

```sh
npm run type-check
```

## Project Structure

- `src/components/ConnectFour.vue` - Main game board component with game logic
- `src/App.vue` - Application container with algorithm explanations
- `src/assets/` - Stylesheets and static assets
- `public/` - Public static files

## Recommended IDE Setup

[VSCode](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur) + [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin).

## Configuration

See [Vite Configuration Reference](https://vitejs.dev/config/) for customization options.
