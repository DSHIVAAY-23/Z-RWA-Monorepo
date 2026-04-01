import type { Config } from 'tailwindcss'

const config: Config = {
  darkMode: "class",
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
        mono: ["JetBrains Mono", "Fira Code", "monospace"],
        space: ["Space Grotesk", "sans-serif"],
      },
      colors: {
        "neon-green": "#00cc66",
        "neon-cyan": "#00d4ff",
        "neon-purple": "#8b5cf6",
        "stellar-amber": "#f59e0b",
      },
      animation: {
        "pulse-slow": "pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite",
        "glow-pulse": "glow 2s ease-in-out infinite alternate",
        "glow-amber": "glowAmber 2s ease-in-out infinite alternate",
        blink: "blink 1s step-end infinite",
      },
      keyframes: {
        glow: {
          from: { boxShadow: "0 0 10px #00cc6644, 0 0 20px #00cc6622" },
          to: { boxShadow: "0 0 20px #00cc6688, 0 0 40px #00cc6644, 0 0 60px #00cc6622" },
        },
        glowAmber: {
          from: { boxShadow: "0 0 10px #f59e0b44, 0 0 20px #f59e0b22" },
          to: { boxShadow: "0 0 20px #f59e0b88, 0 0 40px #f59e0b44, 0 0 60px #f59e0b22" },
        },
        blink: {
          "0%, 100%": { opacity: "1" },
          "50%": { opacity: "0" },
        },
      },
      boxShadow: {
        "neon-green": "0 0 20px #00cc6655, 0 0 40px #00cc6633",
        "neon-cyan": "0 0 20px #00d4ff55, 0 0 40px #00d4ff33",
        "neon-amber": "0 0 20px #f59e0b55, 0 0 40px #f59e0b33",
      },
    },
  },
  plugins: [],
}

export default config
