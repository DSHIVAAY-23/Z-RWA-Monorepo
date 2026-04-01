export default function NetworkBadge() {
  const isMockMode = process.env.NEXT_PUBLIC_MOCK_MODE === 'true';
  const network = process.env.NEXT_PUBLIC_SOLANA_NETWORK || 'devnet';

  return (
    <div className="flex items-center gap-3">
      {isMockMode && (
        <div className="flex items-center gap-1.5 px-3 py-1 rounded-full bg-orange-500/10 border border-orange-500/20 text-orange-400 text-xs font-semibold tracking-wider">
          <span className="relative flex h-2 w-2">
            <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-orange-400 opacity-75"></span>
            <span className="relative inline-flex rounded-full h-2 w-2 bg-orange-500"></span>
          </span>
          DEMO MODE
        </div>
      )}
      <div className="flex items-center gap-1.5 px-3 py-1 rounded-full bg-primary-500/10 border border-primary-500/20 text-primary-400 text-xs font-semibold tracking-wider">
        <span className="w-2 h-2 rounded-full bg-primary-500"></span>
        SOLANA {network.toUpperCase()}
      </div>
    </div>
  );
}
