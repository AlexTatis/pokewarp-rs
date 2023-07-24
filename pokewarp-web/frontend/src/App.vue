<script setup lang="ts">
import Box from './components/Box.vue'
import Party from './components/Party.vue'
import Overview from './components/Overview.vue'
import UploadButton from './components/UploadButton.vue'
import { PKMS } from './data';

</script>

<template>
  <div class="flex pt-2">
    <div class="basis-1/4 flex flex-col gap-8 items-center">
      <h2 class="font-black text-5xl text-center flex gap-3 items-baseline"><UploadButton />Party</h2>
      <Party />
    </div>
    <div class="basis-2/4 flex flex-col gap-8 items-center">
      <h2 class="font-black text-5xl text-center">Boxes</h2>
      <Box :boxes="PKMS" />
    </div>
    <div class="basis-1/4">
      <Overview class="h-full" />
    </div>
  </div>
</template>

<style scoped>
</style>
