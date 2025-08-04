use ftl_sdk::{tools, text, ToolResponse};
use mindkit::{ThinkInput, process_thinking};

tools! {
    /// A detailed tool for dynamic and reflective problem-solving through thoughts. This tool helps analyze problems through a flexible thinking process that can adapt and evolve. Each thought can build on, question, or revise previous insights as understanding deepens.
    ///
    /// When to use this tool:
    /// - Breaking down complex problems into steps
    /// - Planning and design with room for revision
    /// - Analysis that might need course correction
    /// - Problems where the full scope might not be clear initially
    /// - Problems that require a multi-step solution
    /// - Tasks that need to maintain context over multiple steps
    /// - Situations where irrelevant information needs to be filtered out
    ///
    /// Parameters explained:
    /// - thought: Your current thinking step, which can include:
    ///   * Regular analytical steps
    ///   * Revisions of previous thoughts
    ///   * Questions about previous decisions
    ///   * Realizations about needing more analysis
    ///   * Changes in approach
    ///   * Hypothesis generation
    ///   * Hypothesis verification
    /// - nextThoughtNeeded: True if you need more thinking, even if at what seemed like the end
    /// - thoughtNumber: Current number in sequence (can go beyond initial total if needed)
    /// - totalThoughts: Current estimate of thoughts needed (can be adjusted up/down)
    /// - isRevision: A boolean indicating if this thought revises previous thinking
    /// - revisesThought: If is_revision is true, which thought number is being reconsidered
    /// - branchFromThought: If branching, which thought number is the branching point
    /// - branchId: Identifier for the current branch (if any)
    /// - needsMoreThoughts: If reaching end but realizing more thoughts needed
    /// - thoughtType: Type of thought for structured reasoning patterns:
    ///   * "analytical" (default reasoning mode)
    ///   * "critical" (actively looks for flaws and issues)
    ///   * "synthesis" (combines multiple perspectives)
    ///   * "validation" (verifies conclusions)
    /// - confidence: Confidence level in the current reasoning (0.0-1.0)
    /// - customLens: Custom analytical lens to apply (e.g., "Rust best practices", "security")
    ///
    /// Usage examples:
    /// - Express uncertainty: Set confidence: 0.6 when unsure
    /// - Critical analysis: Set thoughtType: "critical" to actively look for flaws
    /// - Domain-specific focus: Set customLens: "security" for security analysis
    /// - Revise previous thought: Set isRevision: true, revisesThought: 2 to revise thought #2
    /// - Branch thinking: Set branchFromThought: 3, branchId: "alternative" to explore from thought 3
    /// - Synthesis mode: Set thoughtType: "synthesis" to combine multiple perspectives
    /// - Validation mode: Set thoughtType: "validation" to verify conclusions
    ///
    /// You should:
    /// 1. Start with an initial estimate of needed thoughts, but be ready to adjust
    /// 2. Feel free to question or revise previous thoughts (use isRevision and revisesThought)
    /// 3. Don't hesitate to add more thoughts if needed, even at the "end"
    /// 4. Express uncertainty when present (use confidence parameter)
    /// 5. Mark thoughts that revise previous thinking or branch into new paths
    /// 6. Ignore information that is irrelevant to the current step
    /// 7. Generate a solution hypothesis when appropriate
    /// 8. Verify the hypothesis based on the Chain of Thought steps
    /// 9. Repeat the process until satisfied with the solution
    /// 10. Provide a single, ideally correct answer as the final output
    /// 11. Only set nextThoughtNeeded to false when truly done and a satisfactory answer is reached
    fn cogitate(input: ThinkInput) -> ToolResponse {
        let result = process_thinking(input);
        text!("{}", result.formatted_output)
    }
}