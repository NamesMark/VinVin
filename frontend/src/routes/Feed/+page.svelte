<script lang="ts">
    import Post from '../../Post.svelte';
    import PostInput from '../../components/PostInput.svelte';
    import { WinePost } from '../../types/types';

    let posts: WinePost[] = [];

    function handlePostSubmit(newPost: WinePost) {
        posts = [newPost, ...posts]; // prepend new post to the list
    }
</script>

<PostInput onSubmit={handlePostSubmit} />

{#each posts as post}
    <Post post={post} />
{/each}