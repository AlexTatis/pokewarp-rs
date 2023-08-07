<script setup lang="ts">
import { useFetch } from '@vueuse/core'
import { PK5, emitter } from '../utils';
import { ArrowUpTrayIcon } from '@heroicons/vue/24/solid'
import { VNodeRef, ref } from 'vue';

const input = ref<VNodeRef>();

async function parseSave(event: Event) {

    const formData = new FormData();

    const files = (<HTMLInputElement>event.target).files;

    if (!files) return;

    formData.append('file', files[0]);

    const { data, statusCode } = await useFetch<PK5[]>('/api/parse', {
        body: formData,
    }).post().json();

    if (statusCode.value != 200) return alert("Error when parsing file");

    if (!data.value) return;

    emitter.emit('saveParsed', data.value);
}

function clickInput() {
    document.getElementById("file-input")?.click()
}

</script>

<template>
    <button @click="clickInput" class="bg-white h-16 p-1 rounded-full flex items-center gap-3 px-3 shadow-md m-1 btn">
        <ArrowUpTrayIcon class="w-8" />
        <p class="font-semibold">Upload</p>
    </button>
    <input type="file" class="file-input w-full max-w-xs file-input-secondary hidden" @change="parseSave" id="file-input">
</template>