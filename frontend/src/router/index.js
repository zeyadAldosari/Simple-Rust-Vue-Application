import { createRouter, createWebHistory } from "vue-router";
import LoginView from "../views/LoginView.vue";
import RegisterView from "../views/RegisterView.vue";

const routes = [
  {
    path: "/",
    redirect: "/login",
  },
  {
    path: "/login",
    name: "Login",
    component: LoginView,
  },
  {
    path: "/register",
    name: "Register",
    component: RegisterView,
  },
  {
    path: "/dashboard",
    name: "Dashboard",
<<<<<<< HEAD
    component: {
      template: `
        <div class="min-h-screen flex items-center justify-center p-4 bg-gray-900 text-white">
          <div class="text-center">
            <h1 class="text-4xl font-bold mb-4">Welcome to your Dashboard!</h1>
            <p class="text-lg text-gray-400">You have successfully logged in.</p>
            <button @click="logout" class="mt-8 py-2 px-6 bg-red-600 hover:bg-red-700 rounded-md text-white font-semibold transition duration-200">
              Log Out
            </button>
          </div>
        </div>
      `,
      methods: {
        logout() {
          this.$router.push("/login");
        },
      },
    },
=======
    component: DashboardView,
    meta: { requiresAuth: true },
>>>>>>> 0466e1d (update index.js)
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

<<<<<<< HEAD
=======
router.beforeEach((to, from, next) => {
  if (to.meta.requiresAuth && !localStorage.getItem("userEmail")) {
    next("/login");
  } else {
    next();
  }
});

>>>>>>> 0466e1d (update index.js)
export default router;
