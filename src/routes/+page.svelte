<script>
  import PcInfo from "./components/pc_info.svelte";
</script>

<div class="top-right">
  <PcInfo />
</div>

<style>

  .top-right {
    position: absolute;
    z-index: 10;
    width: 310px;
    height: 900px;

    background-color: #1a1a1a;
    border-radius: 10px;
    border: 1px solid #2e2e2e;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.4);
    color: #eaeaea;
    font-family: Inter, sans-serif;

  }
</style>
