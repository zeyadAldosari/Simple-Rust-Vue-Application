import { createRouter, createWebHistory } from "vue-router";
import LoginView from "../views/LoginView.vue";
import RegisterView from "../views/RegisterView.vue";
import DashboardView from "../views/DashboardView.vue";

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
    component: DashboardView, // Use the imported component
    meta: { requiresAuth: true }, // Add a meta field to indicate it requires auth
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

// Navigation Guard to check authentication
router.beforeEach((to, from, next) => {
  if (to.meta.requiresAuth && !localStorage.getItem("userEmail")) {
    // If the route requires auth and no user email is found, redirect to login
    next("/login");
  } else {
    // Otherwise, proceed
    next();
  }
});

export default router;
