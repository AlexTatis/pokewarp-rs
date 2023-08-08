<script setup lang="ts">
import { MagnifyingGlassIcon } from '@heroicons/vue/24/solid'
</script>

<template>
    <div class="dropdown dropdown-top flex justify-center">
        <button tabindex="0" class="bg-white h-16 p-1 rounded-full flex items-center gap-3 px-3 shadow-md m-1 btn">
            <MagnifyingGlassIcon class="w-8" />
            <p class="font-semibold">Search</p>
        </button>
        <div tabindex="0" class="dropdown-content z-[1] shadow bg-base-100 rounded-box border p-3">
            <input type="text" class="input input-bordered">
        </div>
    </div>
</template>