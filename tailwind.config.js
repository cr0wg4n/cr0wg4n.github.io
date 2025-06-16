/** @type {import('tailwindcss').Config} */ 
const withMT = require("@material-tailwind/react/utils/withMT");
const colors = require("tailwindcss/colors");

module.exports = withMT({
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
  ],
  darkMode: 'class',
  theme: {
    screens: {
      sm: '480px',
      md: '768px',
      lg: '976px',
      xl: '1440px',
    },
    // colors: {
    //   // ...colors
    //   // 'blue': '#1fb6ff',
    //   // // 'purple': '#7e5bef',
    //   // 'pink': '#ff49db',
    //   // 'orange': '#ff7849',
    //   // 'green': '#13ce66',
    //   // 'yellow': '#ffc82c',
    //   // 'gray-dark': '#273444',
    //   // 'gray': '#8492a6',
    //   // 'gray-light': '#d3dce6',
    //   // 'purple':colors.purple
    //   // 'purple':colors.purple,
    //   // 'teal':colors.teal
    // },
    fontFamily: {
      sans: ['Graphik', 'sans-serif'],
      serif: ['Merriweather', 'serif'],
      ubuntu: ['Ubuntu'],
      quicksand: ['Quicksand'],
      monospace: ['Syne Mono']
    },
    extend: {
      spacing: {
        '128': '32rem',
        '144': '36rem',
      },
      borderRadius: {
        '4xl': '2rem',
      },
      // colors:{
      //   // ...colors
      // }
    }
  },
  plugins: [
    require('@tailwindcss/line-clamp')
  ],
})