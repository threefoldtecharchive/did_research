<template>
  <v-app id="inspire">
    <v-content>
      <v-container
        class="fill-height"
        fluid
      >
        <v-row
          align="center"
          justify="center"
        >
          <v-col
            cols="12"
            sm="8"
            md="4"
          >
            <v-card class="elevation-12">
              <v-toolbar
                flat
              >
                <v-toolbar-title>Register using your DID</v-toolbar-title>
                <v-spacer />
              </v-toolbar>
              <v-card-text>
                <v-form>
                  <v-text-field
                    label="Login"
                    name="login"
                    type="text"
                    v-model="did"
                  />

                  <v-text-field
                    id="password"
                    label="Password"
                    name="password"
                    type="password"
                    v-model="password"
                  />
                </v-form>
              </v-card-text>
              <v-card-actions>
                <v-spacer />
                <v-btn  v-on:click="this.register" color="primary">Register</v-btn>
              </v-card-actions>
            </v-card>
          </v-col>
        </v-row>
      </v-container>
    </v-content>
  </v-app>
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

      const DID_RESOLVER_SERVICE = "http://localhost:3000/"
      axios.get(DID_RESOLVER_SERVICE + this.did).then(response => {
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

        axios.get(`${provider}/${proof}`).then(response => {
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
