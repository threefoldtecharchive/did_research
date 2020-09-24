<template>
  <div class="home">
    <h2>Home</h2>

    <div v-if="isKyced">
      <p>Welcome: {{ this.kycInfo }}</p>
      <p>
        You are currently <b>logged in</b>. Click
        <a href="#" @click="logout">here</a> to logout!.
      </p>
      <h3>Website content here!</h3>
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
export default {
  name: "Home",
  data: () => {
    return {
      isKyced: false,
      kycInfo: null
    };
  },
  mounted() {
    if (localStorage.getItem("kyc")) {
      this.isKyced = true;
      this.kycInfo = JSON.parse(localStorage.getItem("kyc"))["first_name"];
    }
  },
  methods: {
    logout() {
      this.isKyced = false;
      localStorage.removeItem("kyc");
    }
  }
};
</script>
