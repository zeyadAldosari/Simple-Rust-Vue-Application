# Frontend

This is the frontend for the project, built with Vue 3, Vite, Vue Router, Tailwind CSS, and Axios.

## Project Structure

- `src/` – Main source code
  - `views/` – Page components (e.g., Login, Register, Dashboard)
  - `components/` – Reusable UI components
  - `api/` – API integration (e.g., auth.js)
  - `router/` – Vue Router configuration
  - `assets/` – Static assets
  - `style.css` – Global styles (Tailwind CSS)
- `public/` – Static public files
- `index.html` – Main HTML entry point

## Getting Started

1. Install dependencies:
   ```bash
   npm install
   ```
2. Run the development server:
   ```bash
   npm run dev
   ```
3. Build for production:
   ```bash
   npm run build
   ```

## Tech Stack
- [Vue 3](https://vuejs.org/)
- [Vite](https://vitejs.dev/)
- [Vue Router](https://router.vuejs.org/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Axios](https://axios-http.com/)

## Notes
- Uses `<script setup>` syntax for Vue SFCs.
- See the main project README for backend setup and integration.
