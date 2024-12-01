<script lang="ts">
  import { onMount } from "svelte";
  import { createChildWebview } from "$lib/utils";

  onMount(async () => {
    await createChildWebview("mal", "https://myanimelist.net/");
  });
</script>
