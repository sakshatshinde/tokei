<script lang="ts">
  import { onMount } from "svelte";
  import { createChildWebview2 } from "$lib/utils";

  onMount(async () => {
    await createChildWebview2("qbittorrent", "localhost:8888/");
  });
</script>
