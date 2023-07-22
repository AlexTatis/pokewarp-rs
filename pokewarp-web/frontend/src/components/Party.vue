<script setup lang="ts">
import { ref } from 'vue';
import PartySlot from './PartySlot.vue'
import draggable from 'vuedraggable'
import { PK5 } from '../utils'

const PROPS = defineProps<{
    party: PK5[]
}>();

let party = ref(Array.from(PROPS.party)) // Is this fine?
const draggedContext = ref();
const relatedContext = ref();
const receiverId = ref("");


function onMove(e: any) {
    draggedContext.value = e.draggedContext
    relatedContext.value = e.relatedContext
    receiverId.value = e.to.id  // We store the id in a variable, as the onEnd event sets the receiver to the sender


    if(receiverId.value != "party") return false;

}

function onEnd(e: any) {

    if(receiverId.value == "party") return true;

    relatedContext.value.component.alterList((list: PK5[]) => { list[relatedContext.value.index] = draggedContext.value.element  })
    // We use e.newIndex in case the user drags through the other Slots and the index needs to be updated
    party.value[e.newIndex] = relatedContext.value.element  

}

</script>

<template>
    <draggable id="party" v-model="party" class="flex flex-col gap-3" :animation="300" group="pkm" :move="onMove" @end="onEnd">
        <template #item="{ element: pkm }">
            <PartySlot :pkm="pkm" />
        </template>
    </draggable>
</template>