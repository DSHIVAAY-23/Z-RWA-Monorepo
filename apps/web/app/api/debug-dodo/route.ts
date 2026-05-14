export async function GET() {
  const response = await fetch('https://api.dodopayments.com/payments?limit=5', {
    headers: { 'Authorization': `Bearer ${process.env.DODO_API_KEY}` }
  });
  const data = await response.json();
  return Response.json(data);
}
