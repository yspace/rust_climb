<script>
	import Router from 'svelte-spa-router';
	import routes from './routes';
</script>

<nav>
	<ul>
		<li>
			<a href="/">index</a>
		</li>
		<li>
			<a href="/#/lorem/4">lorem</a>
		</li>
		<li>
			<a href="/#/about">about</a>
		</li>
	</ul>
</nav>
<div class="bg-blue-600">
hi can you see me
</div>
<main>
	 
	<Router {routes} />
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
