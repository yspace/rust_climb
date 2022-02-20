<script>
	import Router from 'svelte-spa-router';
// import Renderrer from './components/Renderrer.svelte';
	import routes from './routes';

	import {link} from 'svelte-spa-router'

	import "carbon-components-svelte/css/all.css";

	let theme = "white"; // "white" | "g10" | "g80" | "g90" | "g100"

    $: document.documentElement.setAttribute("theme", theme);
</script>

<nav>
	<ul>
		<li>
			<a href="/">index</a>
		</li>
		<li>
			<a href="/#/md">md</a>
		</li>
		<li>
			<a href="/#/lorem/4">lorem</a>
		</li>
		<li>
			<a href="/#/about">about</a>
		</li>
		<li>
			<a href="/carbon-demo" use:link>haha</a>
		</li>
	</ul>
</nav>
<div class="bg-blue-600">
hi can you see me
</div>


<!-- <div>
	temporary contentWindow
	<Renderrer title="Hello" content="world" />
</div> -->

<main>
	 
	<Router {routes} />
</main>

<style>
	/* main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	} */
</style>
