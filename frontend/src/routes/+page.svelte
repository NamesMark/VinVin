<!-- YOU CAN DELETE EVERYTHING IN THIS PAGE -->
<script lang = "ts">
	//import type { WinePost } from '../types/types';
	import Post from '../components/Post.svelte';
	import PostInput from '../components/PostInput.svelte';

	let posts: never[] = [];

	fetch("/posts")
		.then(res => res.json())
		.then(data => {
			posts = data;
		});
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-5">
		<h1 class="h1">Let's get cracking bones!</h1>
		<p>Start by exploring:</p>
		<ul>
			<li><code class="code">/src/routes/+layout.svelte</code> - barebones layout</li>
			<li><code class="code">/src/app.postcss</code> - app wide css</li>
			<li>
				<code class="code">/src/routes/+page.svelte</code> - this page, you can replace the contents
			</li>
		</ul>

		{#each posts as post}
			<Post post={post} />
		{/each}

	</div>
</div>
