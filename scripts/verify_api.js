const axios = require('axios');

async function verifyApi() {
  console.log("🚀 Starting Z-RWA API Verification...");
  
  const payload = {
    age: 25,
    panHash: "0x3ba505c657aad8f", // Ensure 0x prefix for hex
    kycScore: 750,
    walletAddress: "GsPrDLXoqVbcWwofYpRZFJg4h5dzHEjyNfPyzPrcUKGd"
  };

  try {
    console.log("Sending request to /api/generate-proof...");
    const response = await axios.post('http://localhost:3000/api/generate-proof', payload);
    
    if (response.status === 200 && response.data.proof) {
      console.log("✅ Success! Proof generated.");
      console.log("Proof ID:", response.data.proofId);
      console.log("Public Signals:", response.data.publicSignals);
    } else {
      console.log("❌ Failed! Unexpected response:", response.status, response.data);
    }
  } catch (error) {
    console.error("❌ Error calling API:");
    if (error.response) {
      console.error("Status:", error.response.status);
      console.error("Data:", error.response.data);
    } else {
      console.error(error.message);
    }
    console.log("\n💡 Possible fix: Ensure the local server is running (npm run dev) and the panHash has a '0x' prefix.");
  }
}

verifyApi();
