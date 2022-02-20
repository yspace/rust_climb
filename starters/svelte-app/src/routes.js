import Home from './routes/Home.svelte';
import Lorem from './routes/Lorem.svelte';
import NotFound from './routes/NotFound.svelte';
import About from './routes/About.svelte';
import CarbonDemo from './routes/CarbonDemo.svelte';


export default {
    '/': Home,
    '/about': About,
    '/lorem/:repeat': Lorem,
    '/carbon-demo': CarbonDemo,
    // The catch-all route must always be last
    '*': NotFound
};
