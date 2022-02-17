module.exports = {
    plugins: [
        require('autoprefixer')({
            overrideBrowserslist: ['defaults and last 4 versions'],
        }),
        require('postcss-import')(),
         // Keep current plugins and add tailwind below:
         require('tailwindcss')(),
    ],
};
