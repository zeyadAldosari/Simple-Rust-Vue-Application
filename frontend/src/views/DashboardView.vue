<template>
  <div class="min-h-screen flex items-center justify-center p-4">
    <div
      class="relative w-full max-w-md mx-auto p-8 rounded-lg shadow-2xl overflow-hidden bg-gray-800 backdrop-blur-md"
    >
      <div
        class="absolute inset-0 bg-gradient-to-br from-purple-700 via-pink-600 to-orange-500 opacity-20 blur-xl"
      ></div>

      <div class="relative z-10 text-center">
        <h2 class="text-4xl font-extrabold text-white mb-4 leading-tight">
          Welcome, <br />
          <span
            class="text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-pink-400"
          >
            {{ userEmail ? userEmail.split("@")[0] : "User" }} </span
          >!
        </h2>

        <button
          @click="handleLogout"
          class="w-full flex justify-center items-center py-3 px-4 rounded-md text-white font-semibold shadow-lg transition duration-300 ease-in-out transform hover:scale-105 bg-gradient-to-r from-red-600 to-rose-600 hover:from-red-700 hover:to-rose-700"
        >
          <svg
            class="w-5 h-5 mr-2"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
            ></path>
          </svg>
          Log Out
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const userEmail = ref(null);

const handleLogout = () => {
  localStorage.removeItem("userEmail");
  router.push("/login");
};

onMounted(() => {
  const storedEmail = localStorage.getItem("userEmail");
  if (storedEmail) {
    userEmail.value = storedEmail;
  } else {
    router.push("/login");
  }
});
</script>

<style scoped>
</style>
