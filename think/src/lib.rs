// Copyright 2025 Corey Ryan
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ftl_sdk::{ToolResponse, text, tools};
use mindkit::{ThinkInput, process_thinking};

tools! {
    /// A structured reasoning tool for dynamic and reflective problem-solving. This tool helps analyze problems through a flexible reasoning framework that can adapt and evolve. Each reasoning step can build on, question, or revise previous insights as understanding deepens.
    ///
    /// When to use this tool:
    /// - Breaking down complex problems into structured reasoning steps
    /// - Planning and design with room for revision and refinement
    /// - Analysis that might need course correction or alternative approaches
    /// - Problems where the full scope might not be clear initially
    /// - Problems that require systematic multi-step reasoning
    /// - Tasks that need to maintain context across reasoning steps
    /// - Situations where irrelevant information needs to be filtered out
    ///
    /// Parameters explained:
    /// - thought: Your current reasoning step, which can include:
    ///   * Regular analytical reasoning
    ///   * Revisions of previous reasoning steps
    ///   * Questions about previous decisions or assumptions
    ///   * Realizations about needing deeper analysis
    ///   * Changes in reasoning approach
    ///   * Hypothesis generation and testing
    ///   * Solution verification
    /// - `next_thought_needed`: True if you need more reasoning steps, even if at what seemed like the end
    /// - `thought_number`: Current step number in sequence (can go beyond initial total if needed)
    /// - `total_thoughts`: Current estimate of reasoning steps needed (can be adjusted up/down)
    /// - `is_revision`: A boolean indicating if this step revises previous reasoning
    /// - `revises_thought`: If `is_revision` is true, which step number is being reconsidered
    /// - `branch_from_thought`: If branching, which step number is the branching point
    /// - `branch_id`: Identifier for the current branch (if any)
    /// - `needs_more_thoughts`: If reaching end but realizing more reasoning steps needed
    /// - `thought_type`: Type of reasoning for structured analytical patterns:
    ///   * "analytical" (default reasoning mode)
    ///   * "critical" (actively looks for flaws and issues)
    ///   * "synthesis" (combines multiple perspectives)
    ///   * "validation" (verifies conclusions)
    /// - `confidence`: Confidence level in the current reasoning (0.0-1.0)
    /// - `custom_lens`: Custom analytical lens to apply (e.g., "Rust best practices", "security")
    ///
    /// Usage examples:
    /// - Express uncertainty: Set `confidence`: 0.6 when unsure
    /// - Critical analysis: Set `thought_type`: "critical" to actively look for flaws
    /// - Domain-specific focus: Set `custom_lens`: "security" for security analysis
    /// - Revise previous thought: Set `is_revision`: true, `revises_thought`: 2 to revise thought #2
    /// - Branch thinking: Set `branch_from_thought`: 3, `branch_id`: "alternative" to explore from thought 3
    /// - Synthesis mode: Set `thought_type`: "synthesis" to combine multiple perspectives
    /// - Validation mode: Set `thought_type`: "validation" to verify conclusions
    ///
    /// You should:
    /// 1. Start with an initial estimate of needed reasoning steps, but be ready to adjust
    /// 2. Feel free to question or revise previous reasoning (use `is_revision` and `revises_thought`)
    /// 3. Don't hesitate to add more reasoning steps if needed, even at the "end"
    /// 4. Express uncertainty when present (use confidence parameter)
    /// 5. Mark steps that revise previous reasoning or branch into new analytical paths
    /// 6. Ignore information that is irrelevant to the current reasoning step
    /// 7. Generate solution hypotheses when appropriate
    /// 8. Verify hypotheses based on your structured reasoning chain
    /// 9. Repeat the process until satisfied with the analytical conclusion
    /// 10. Provide a single, well-reasoned answer as the final output
    /// 11. Only set `next_thought_needed` to false when reasoning is complete and conclusion is satisfactory
    fn structured_reasoning(input: ThinkInput) -> ToolResponse {
        let result = process_thinking(input);
        text!("{}", result.formatted_output)
    }

    // Note: implementation_planner and requirements_analyzer are not available
    // in the published mindkit crate yet. They will be added in a future version.
}
