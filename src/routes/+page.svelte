<script>
  import PcInfo from "../lib/components/pc_info.svelte";
  import Activity from "../lib/components/activity.svelte";
  import VoiceRecog from "../lib/components/voice_recog.svelte";
</script>

<div class="sidebar">
  <PcInfo />
  <Activity />
  <VoiceRecog />
</div>

<style>

  .sidebar {
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
