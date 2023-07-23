<script setup lang="ts">
import { useFetch } from '@vueuse/core'
import { PK5, emitter } from '../utils';
import { PlusIcon } from '@heroicons/vue/24/solid'
import { VNodeRef, ref } from 'vue';

const input = ref<VNodeRef>();

async function parseSave(event: Event) {

    const formData = new FormData();

    const files = (<HTMLInputElement>event.target).files;

    if (!files) return;

    formData.append('file', files[0]);

    const { data } = await useFetch<PK5[]>('/api/parse', {
        body: formData,
    }).post().json();

    
    if (!data.value) return;
    
    emitter.emit('saveParsed', data.value);
}

function clickInput() {
    document.getElementById("file-input")?.click()
}

</script>

<template>
    <div class="bg-primary mt-3 mr-3 px-2 py-4 text-white rounded-md flex gap-4">
        <button @click="clickInput" class="btn btn-square"><PlusIcon class="w-8" /></button>
        <button @click="clickInput" class="btn btn-square"><PlusIcon class="w-8" /></button>
        <button @click="clickInput" class="btn btn-square"><PlusIcon class="w-8" /></button>
        <button @click="clickInput" class="btn btn-square"><PlusIcon class="w-8" /></button>
        <button @click="clickInput" class="btn btn-square"><PlusIcon class="w-8" /></button>
        <input type="file" class="file-input w-full max-w-xs file-input-secondary hidden" @change="parseSave" id="file-input">
    </div>
</template>