import Home from './routes/Home.svelte';
import Lorem from './routes/Lorem.svelte';
import NotFound from './routes/NotFound.svelte';
import About from './routes/About.svelte';

export default {
    '/': Home,
    '/about': About,
    '/lorem/:repeat': Lorem,
    // The catch-all route must always be last
    '*': NotFound
};
