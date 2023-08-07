<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { PK5, emitter } from '../utils'
import { useWindowSize } from '@vueuse/core'

const PROPS = defineProps<{
    pkm?: PK5
}>();

const selected = ref(false);

watch(selected, (watched: boolean) => {
    if (!PROPS.pkm) return;


    if (watched) {
        emitter.emit('setOverview', PROPS.pkm)
        emitter.emit('newSlotSelected')
        emitter.on('newSlotSelected', newSlotSelectedHandler)
        return;
    }

    emitter.off('newSlotSelected', newSlotSelectedHandler)

})

function newSlotSelectedHandler() {
    selected.value = false
}

const isMobileViewport = computed(() => useWindowSize().width.value <= 640)


</script>
<template>
    <div class="md:w-32 md:h-32 h-24 w-24 shadow-lg rounded-lg bg-white/30"
        :class="{ ' border-red-500 border-dashed border-2': selected, 'no-drag': PROPS.pkm && PROPS.pkm.id == 0 || isMobileViewport }"
        @click="selected = !selected" onclick="if (window.innerWidth <= 640) my_modal_5.showModal()">
        <div v-if="PROPS.pkm && PROPS.pkm.id != 0" class="flex items-center h-full pixelated gap-2">
            <img :src="`/pokesprite/pokemon-gen8/regular/${PROPS.pkm.species.toLowerCase()}.png`" class="w-32"
                :class="{ 'animate-bounce': selected }">
        </div>
    </div></template>
