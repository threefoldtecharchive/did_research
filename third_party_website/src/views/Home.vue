<template>
  <div class="home">
    <h2>Home</h2>

    <div v-if="kycInfo">
      <p>
        You are currently <b>logged in</b>. Click
        <a href="#" @click="logout">here</a> to logout!.
      </p>
      <h3>Personal data: </h3>
       <vue-json-pretty :data="kycInfo" />
        <a @click="logout">Logout</a>
    </div>
    <div v-else>
      <p>
        You are currently <b>not</b> logged in. Click
        <a href="/register">here</a> to login/register.
      </p>
    </div>
  </div>
</template>

<script>
import VueJsonPretty from 'vue-json-pretty'
export default {
  name: "Home",
  data: () => {
    return {
      kycInfo: null
    };
  },
  components: {
    VueJsonPretty
  },
  mounted() {
    if (localStorage.getItem("kyc")) {
      this.kycInfo = JSON.parse(localStorage.getItem("kyc"))
    }
  },
  methods: {
    logout() {
      localStorage.removeItem("kyc")
      this.$router.push('/register')
    }
  }
};
</script>
