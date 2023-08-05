<script setup lang="ts">
import Box from './components/Box.vue'
import Party from './components/Party.vue'
import Overview from './components/Overview.vue'
import UploadButton from './components/UploadButton.vue'
import { PKMS } from './data';

</script>

<template>
  <div class="md:flex pt-2">
    <div class="basis-1/4 md:flex flex-col gap-8 items-center hidden">
      <h2 class="font-black text-5xl text-center flex gap-3 items-baseline">
        <UploadButton />Party
      </h2>
      <Party />
    </div>
    <div class="basis-2/4 flex flex-col gap-8 items-center">
      <h2 class="font-black text-5xl text-center flex gap-3 items-baseline">
        <div class="md:hidden">
          <UploadButton />
        </div>
        Boxes
      </h2>
      <Box :boxes="PKMS" />
    </div>
    <div class="basis-1/4">
      <Overview class="h-full hidden md:block" />
    </div>
    <dialog id="my_modal_5" class="modal modal-bottom sm:modal-middle md:hidden">
      <form method="dialog" class="modal-box p-0">
        <Overview />
      </form>
      <form method="dialog" class="modal-backdrop">
        <button
          class="btn btn-sm btn-circle btn-ghost absolute text-white bg-primary mb-10 top-0 bottom-0 mt-14 left-1/2 transform -translate-x-1/2 -translate-y-1/2">✕</button>
      </form>
    </dialog>
  </div>
</template>

<style scoped></style>
