<script setup lang="ts">
import { ref, watch } from 'vue';
import { PK5, emitter } from '../utils'

const PROPS = defineProps<{
    pkm?: PK5
}>();

const selected = ref(false);

watch(selected, (watched: boolean) => {
    if(!PROPS.pkm) return;
    
    
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


</script>
<template>
    <div class="w-32 h-32 shadow-lg rounded-lg bg-white" :class="{' border-red-500 border-dashed border-2': selected}" @click="selected = !selected">
        <div v-if="PROPS.pkm && PROPS.pkm.id != 0" class="flex items-center h-full pixelated gap-2">
            <img :src="`/pokesprite/pokemon-gen8/regular/${PROPS.pkm.species.toLowerCase()}.png`" class="w-32" :class="{'animate-bounce': selected}">
        </div>
    </div>
</template>
