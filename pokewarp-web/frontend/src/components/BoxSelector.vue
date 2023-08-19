<script setup lang="ts">
import { ref } from 'vue';

const PROPS = defineProps<{
    maxBoxes: number,
    currentBox: number
}>();

const emit = defineEmits<{
    (event: "update:currentBox", box: number): number
}>()

const currentBox = ref(PROPS.currentBox)

function previousBox() { if (currentBox.value > 0) {currentBox.value--; emit('update:currentBox', currentBox.value)} }
function nextBox() { if (currentBox.value < PROPS.maxBoxes - 1) {currentBox.value++; emit('update:currentBox', currentBox.value)} }


</script>

<template>
    <div class="rounded-full shadow w-52 bg-white p-2 flex items-center justify-between h-16">
        <div class="bg-[#303030] w-4 h-4 mask mask-triangle-3 btn btn-ghost btn-square" @click="previousBox" />
        <h2 class="font-black text-4xl text-center">Box {{ currentBox + 1 }}</h2>
        <div class="bg-[#303030] w-4 h-4 mask mask-triangle-4 btn btn-ghost btn-square" @click="nextBox" />
      </div>
</template>