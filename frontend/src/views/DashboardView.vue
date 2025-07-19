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
            {{ userEmail ? userEmail.split("@")[0] : "Voyager!" }} </span
          >!
        </h2>
        <p class="text-lg text-gray-300 mb-8">
          You've successfully embarked on your journey.
        </p>

        <div class="space-y-4 mb-8">
          <button
            @click="navigateToProfile"
            class="w-full flex justify-center items-center py-3 px-4 rounded-md text-white font-semibold shadow-lg transition duration-300 ease-in-out transform hover:scale-105 bg-gradient-to-r from-blue-600 to-cyan-600 hover:from-blue-700 hover:to-cyan-700"
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
                d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
              ></path>
            </svg>
            View Profile
          </button>

          <button
            @click="navigateToSettings"
            class="w-full flex justify-center items-center py-3 px-4 rounded-md text-white font-semibold shadow-lg transition duration-300 ease-in-out transform hover:scale-105 bg-gradient-to-r from-green-600 to-teal-600 hover:from-green-700 hover:to-teal-700"
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
                d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.542.33 1.15.495 1.724 1.066z"
              ></path>
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
              ></path>
            </svg>
            App Settings
          </button>
        </div>

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
import { ref, onMounted } from "vue"; // Import onMounted
import { useRouter } from "vue-router";

const router = useRouter();
const userEmail = ref(null); // Reactive variable to hold the user's email

// Function to handle logout
const handleLogout = () => {
  localStorage.removeItem("userEmail"); // Clear the user's email from local storage
  router.push("/login");
};

// Example navigation functions (unchanged)
const navigateToProfile = () => {
  alert("Navigating to Profile (route not implemented)");
};

const navigateToSettings = () => {
  alert("Navigating to Settings (route not implemented)");
};

// Lifecycle hook to run code when the component is mounted
onMounted(() => {
  const storedEmail = localStorage.getItem("userEmail");
  if (storedEmail) {
    userEmail.value = storedEmail;
  } else {
    // If no email is found, redirect to login
    router.push("/login");
  }
});
</script>

<style scoped>
</style>
