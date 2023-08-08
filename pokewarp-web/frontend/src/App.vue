<script setup lang="ts">
import Boxes from './components/Boxes.vue'
import Party from './components/Party.vue'
import Overview from './components/Overview.vue'
import UploadButton from './components/UploadButton.vue'
import Avatar from './components/Avatar.vue'
import SearchButton from './components/SearchButton.vue'


</script>

<template>
  <div class="hidden md:block">
    <div class="h-screen w-80 bg-gray-800 absolute -left-32 -skew-x-12" />
    <div class="h-screen w-64 bg-red-500 absolute -left-28 -skew-x-12" />
  </div>
  <div class="md:flex pt-2 z-10 relative">
    <div class="left-0 basis-1/4 md:flex flex-col gap-6 items-center hidden">
      <h2 class="font-black text-4xl text-center rounded-full shadow w-52 bg-white p-2 h-16">Party</h2>
      <Party />
    </div>
    <div class="basis-2/4 flex flex-col gap-8 items-center">
      <Boxes />
      <div class="sticky bottom-2 flex items-center gap-3">
        <Avatar />
        <UploadButton />
        <SearchButton />
        <!-- <label for="my-drawer" class="btn bg-white shadow-md btn-ghost h-16 rounded-full drawer-button md:hidden">Open party</label> -->
      </div>
    </div>
    <div class="basis-1/4">
      <Overview class="h-full hidden md:block" />
    </div>

    <!--SMALL VIEWPORT VERSIONS-->

    <dialog id="my_modal_5" class="modal modal-bottom sm:modal-middle md:hidden">
      <form method="dialog" class="modal-box p-0">
        <Overview />
      </form>
      <form method="dialog" class="modal-backdrop">
        <button>close</button>
      </form>
    </dialog>

    <div class="drawer md:hidden">
      <input id="my-drawer" type="checkbox" class="drawer-toggle" />
      <div class="drawer-content">
      </div>
      <div class="drawer-side">
        <label for="my-drawer" class="drawer-overlay"></label>
        <div class="h-full flex items-center pl-1">
          <Party />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
