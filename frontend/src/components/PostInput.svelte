<script lang = "ts">
    export let onSubmit;

    type WinePostInput = {
        wine_name: string;
        wine_year?: number;
        region?: string;
        look?: string;
        nose?: string;
        palate?: string;
        tail?: string;
    };

    let wine_name: string = "";
    let wine_year: number;
    let region: string;
    let look: string;
    let nose: string;
    let palate: string;
    let tail: string;

    function handleSubmit() {
        const postData: WinePostInput = {
            wine_name,
            wine_year,
            region,
            look,
            nose,
            palate,
            tail
        };
        onSubmit(postData);
    }
</script>

<!-- Simple form for user input -->
<form on:submit|preventDefault={handleSubmit}>
    <label>
        Wine Name:
        <input type="text" bind:value={wine_name} />
    </label>
    <br />

    <label>
        Wine Year:
        <input type="number" bind:value={wine_year} />
    </label>
    <br />

    <label>
        Region:
        <input type="text" bind:value={region} />
    </label>
    <br />

    <label>
        Look:
        <textarea bind:value={look}></textarea>
    </label>
    <br />

    <label>
        Nose:
        <textarea bind:value={nose}></textarea>
    </label>
    <br />

    <label>
        Palate:
        <textarea bind:value={palate}></textarea>
    </label>
    <br />

    <label>
        Tail:
        <textarea bind:value={tail}></textarea>
    </label>
    <br />

    <button type="submit">Submit</button>
</form>

<!-- Display entered details -->
<section>
    <h2>Post Details</h2>
    <p><strong>Name:</strong> {wine_name}</p>
    <p><strong>Year:</strong> {wine_year}</p>
    <p><strong>Region:</strong> {region}</p>
    <p><strong>Look:</strong> {look}</p>
    <p><strong>Nose:</strong> {nose}</p>
    <p><strong>Palate:</strong> {palate}</p>
    <p><strong>Tail:</strong> {tail}</p>
</section>

<style>
    label {
        display: block;
        margin: 1em 0;
    }
</style>