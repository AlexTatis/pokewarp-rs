<script setup lang="ts">
import { ref } from 'vue';
import BoxSlot from './BoxSlot.vue'
import draggable from 'vuedraggable'
import { EMPTY_PK5, PK5 } from '../utils'


const PROPS = defineProps<{
    boxes: PK5[]
}>();

let boxes = ref(Array.from(PROPS.boxes)) // Is this fine?
const BOXES_LENGTH = boxes.value.length;
const draggedContext = ref();
const relatedContext = ref();
const receiverId = ref("");

for(let i = 0; i < 30 - BOXES_LENGTH; i++) {
    boxes.value.push(EMPTY_PK5)
}

function onMove(e: any) {
    draggedContext.value = e.draggedContext
    relatedContext.value = e.relatedContext
    receiverId.value = e.to.id  // We store the id in a variable, as the onEnd event sets the receiver to the sender

    return false;   // We always return false so we keep the swapping behaviour even inside the Box

}

function onEnd() {

    relatedContext.value.component.alterList((list: PK5[]) => { list[relatedContext.value.index] = draggedContext.value.element  })
    boxes.value[draggedContext.value.index] = relatedContext.value.element

}

</script>

<template>
    <draggable id="box" v-model="boxes" class="grid grid-cols-6 grid-rows-5 gap-5" group="pkm" :move="onMove" @end="onEnd">
        <template #item="{ element: pkm }">
            <BoxSlot :pkm="pkm" />
        </template>
    </draggable>
</template>