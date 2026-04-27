import { NextResponse } from "next/server";
import { getOptimalPriorityFee } from "../../../lib/quicknode";

export async function GET() {
  const fee = await getOptimalPriorityFee();
  return NextResponse.json(
    { fee, unit: "microlamports", source: "QuickNode" },
    { 
      headers: { 
        "Cache-Control": "public, max-age=10",
        "Access-Control-Allow-Origin": "*"
      } 
    }
  );
}
