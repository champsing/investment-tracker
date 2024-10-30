<script setup lang="ts">
import { Account } from '@/composables/account';
import { Ref, ref } from 'vue';
import Details from './investments/Details.vue';
import Accounts from './investments/Accounts.vue';

const account: Ref<Account> = ref(null)
</script>

<template>
    <template v-if="account == null">
        <Accounts @select="(value) => account = value" />
    </template>
    <template v-else>
        <Details :account="account" @back="() => account = null" />
    </template>
</template>

<style scoped></style>
