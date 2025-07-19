<template>
  <div
    class="relative w-full max-w-md mx-auto p-8 rounded-lg shadow-2xl overflow-hidden bg-gray-800 backdrop-blur-md"
  >
    <div
      class="absolute inset-0 bg-gradient-to-br from-purple-700 via-pink-600 to-orange-500 opacity-20 blur-xl"
    ></div>
    <div class="relative z-10">
      <h2 class="text-3xl font-bold text-center text-white mb-6">
        {{ title }}
      </h2>
      <form @submit.prevent="$emit('submit')">
        <slot></slot>
        <button
          type="submit"
          :disabled="loading"
          class="w-full flex justify-center items-center py-3 px-4 rounded-md text-white font-semibold shadow-lg transition duration-300 ease-in-out transform hover:scale-105 bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <svg
            v-if="loading"
            class="animate-spin -ml-1 mr-3 h-5 w-5 text-white"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            ></circle>
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            ></path>
          </svg>
          {{ buttonText }}
        </button>
      </form>
      <p v-if="errorMessage" class="mt-4 text-center text-red-400">
        {{ errorMessage }}
      </p>
      <p v-if="successMessage" class="mt-4 text-center text-green-400">
        {{ successMessage }}
      </p>
      <div class="mt-6 text-center">
        <slot name="footer"></slot>
      </div>
    </div>
  </div>
</template>

<script setup>
import { defineProps, defineEmits } from "vue";

defineProps({
  title: {
    type: String,
    required: true,
  },
  buttonText: {
    type: String,
    required: true,
  },
  loading: {
    type: Boolean,
    default: false,
  },
  errorMessage: {
    type: String,
    default: "",
  },
  successMessage: {
    type: String,
    default: "",
  },
});

defineEmits(["submit"]);
</script>
