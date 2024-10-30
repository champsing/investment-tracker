<script setup lang="ts">
import { Account } from '@/composables/account';

const emits = defineEmits<{ back: [] }>()
const props = defineProps<{ account: Account }>()

console.log(emits)
</script>

<template>
    <div>
        {{ props.account }}
    </div>
</template>

<style scoped></style>
