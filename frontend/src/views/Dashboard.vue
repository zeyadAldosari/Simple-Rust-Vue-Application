<template>
  <div class="bg-white p-8 rounded-lg shadow-md w-full max-w-md text-center">
    <h2 class="text-3xl font-bold text-gray-800 mb-4">
      Welcome to the Dashboard!
    </h2>
    <p class="text-gray-600 mb-6">You have successfully logged in.</p>
    <button
      @click="handleLogout"
      class="bg-red-600 hover:bg-red-700 text-white font-bold py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-opacity-50 transition duration-200"
    >
      Logout
    </button>
  </div>
</template>

<script setup>
import { useRouter } from "vue-router";

const router = useRouter();

const handleLogout = () => {
  localStorage.removeItem("userToken");
  router.push("/login");
};
</script>

<style scoped>
</style>
