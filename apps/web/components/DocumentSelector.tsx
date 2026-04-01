"use client";

import { useState } from "react";

export const DOC_OPTIONS = [
    { id: "aadhaar", name: "Aadhaar Card (UIDAI)", requirement: "Requires verifiable QR snippet." },
    { id: "pan", name: "PAN Card (Income Tax Dept)", requirement: "Requires name & DOB matching." },
    { id: "passport", name: "Indian Passport", requirement: "Requires MRZ decoding." },
    { id: "bhulekh", name: "Land Record (Digital Bhulekh)", requirement: "Verify State Land Registry signature." },
    { id: "investor", name: "Accredited Investor Certificate", requirement: "Requires SEBI registered CA sign." },
];

interface DocumentSelectorProps {
    selectedDocId: string;
    onDocIdChange: (val: string) => void;
    onFileSelected: (file: File | null) => void;
    isRunning: boolean;
}

export default function DocumentSelector({
    selectedDocId,
    onDocIdChange,
    onFileSelected,
    isRunning,
}: DocumentSelectorProps) {
    const selectedDoc = DOC_OPTIONS.find(d => d.id === selectedDocId) || DOC_OPTIONS[0];
    const [fileState, setFileState] = useState<File | null>(null);

    const handleFile = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (e.target.files && e.target.files[0]) {
            setFileState(e.target.files[0]);
            onFileSelected(e.target.files[0]);
        }
    };

    return (
        <div className="mb-8 space-y-4">
            <div>
                <label className="block text-xs font-semibold uppercase tracking-widest text-gray-500 mb-2">
                    Step 1: Select Verifiable Document Type
                </label>
                <div className="relative">
                    <select
                        value={selectedDocId}
                        onChange={(e) => onDocIdChange(e.target.value)}
                        disabled={isRunning || fileState !== null}
                        className="
                            w-full rounded-xl border border-gray-700
                            bg-gray-950
                            pl-4 pr-10 py-3.5
                            font-mono text-sm text-gray-200
                            focus:outline-none focus:border-cyan-500/60 focus:ring-1 focus:ring-cyan-500/20
                            hover:border-gray-600
                            disabled:opacity-50 disabled:cursor-not-allowed
                            transition-all duration-200 appearance-none cursor-pointer
                        "
                    >
                        {DOC_OPTIONS.map((opt) => (
                            <option key={opt.id} value={opt.id}>{opt.name}</option>
                        ))}
                    </select>
                    <div className="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none">
                        <svg className="h-4 w-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
                        </svg>
                    </div>
                </div>
            </div>

            <div className="rounded-lg border border-gray-800 bg-gray-900/50 p-4">
                <p className="text-sm font-medium text-gray-300">
                    <span className="text-cyan-400 font-semibold mr-2">Condition:</span>
                    {selectedDoc.requirement}
                </p>
            </div>

            {!fileState ? (
                <label className="block border-2 border-dashed border-gray-700 rounded-xl p-8 text-center hover:border-green-500/40 hover:bg-green-500/5 transition-all cursor-pointer">
                    <div className="text-3xl mb-3">📄</div>
                    <div className="text-gray-200 font-medium font-sans mb-1">
                        Drop document here or click to browse
                    </div>
                    <div className="text-gray-500 text-xs font-mono">
                        PDF, PNG, JPG (MAX. 10MB)
                    </div>
                    <div className="mt-3 inline-flex items-center gap-1.5 text-[10px] text-green-400 font-mono uppercase tracking-widest bg-green-500/10 px-2 py-1 rounded">
                        <span>🔒</span> Zero data leaves your device
                    </div>
                    <input type="file" className="hidden" accept=".pdf,.png,.jpg,.jpeg" onChange={handleFile} disabled={isRunning} />
                </label>
            ) : (
                <div className="border border-green-500/20 rounded-xl p-4 bg-green-500/5 flex flex-col gap-3">
                    <div className="flex items-center justify-between border-b border-white-10 pb-3">
                        <div className="flex items-center gap-3">
                            <div className="text-2xl h-10 w-10 bg-black/40 rounded flex items-center justify-center border border-gray-800">📄</div>
                            <div>
                                <div className="text-sm font-semibold text-white truncate max-w-[200px]">{fileState.name}</div>
                                <div className="text-xs font-mono text-gray-400">{(fileState.size / 1024 / 1024).toFixed(2)} MB</div>
                            </div>
                        </div>
                        <button onClick={() => { setFileState(null); onFileSelected(null); }} className="text-xs font-semibold text-purple-400 hover:text-purple-300 bg-purple-500/10 px-3 py-1.5 rounded-lg border border-purple-500/20">
                            Remove
                        </button>
                    </div>
                    <div className="flex items-center">
                        <span className="text-[10px] text-gray-500 font-mono uppercase tracking-widest mr-2">Status:</span>
                        <span className="text-xs text-green-400 font-mono">Ready for ZK Proving</span>
                    </div>
                </div>
            )}
        </div>
    );
}
