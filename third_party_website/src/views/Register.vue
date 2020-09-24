<template>
  <div class="home">
    <h2>Register</h2>
    <p>
      Note: The password is just purely for the POC, in reality the app is going
      to sign the object containing the DID to verify it is the user sending the
      data.
    </p>
    <p>
      <label for="did">did </label>
      <input id="did" v-model="did" type="text" name="did" />
    </p>

    <p>
      <label for="password">password </label>
      <input id="password" v-model="password" type="password" name="password" />
    </p>

    <p>
      <input type="submit" value="Submit" @click="register" />
    </p>
  </div>
</template>

<script>
import axios from "axios";

export default {
  name: "Home",
  data: () => {
    return {
      did: "did:tf:5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      password: "abc123"
    };
  },
  methods: {
    register() {
      console.log("Resolving did: ", this.did);

      axios.get("http://127.0.0.1:5000/did/" + this.did).then(response => {
        if (response.status !== 200) {
          console.log("Failed to resolve did.");
          localStorage.removeItem("kyc");
          return;
        }

        console.log("did document: ", response.data);

        console.log("Grabbing KYC information from did document");

        let provider = response.data.kyc[0].provider;
        let proof = response.data.kyc[0].proof;

        console.log("KYC provider: ", provider);
        console.log("KYC proof: ", proof);

        // This can be accomplished with asking the service or asking for their public key and verifying the sign urself.
        console.log("Validating if KYC proof is signed by given provider");

        axios.get(provider + proof).then(response => {
          if (response.status !== 200) {
            console.log(
              "Failed to validate KYC proof with the given provider."
            );
            localStorage.removeItem("kyc");
            return;
          }

          console.log("KYC validation succeeded!");
          console.log("KYC validation: ", response.data);

          localStorage.setItem("kyc", JSON.stringify(response.data));

          console.log("Going to redirect in 3 seconds");

          setTimeout(() => {
            this.$router.push("/");
          }, 3000);
        });
      });
    }
  }
};
</script>
