# Problem

## Overview
**Purpose**: To define the specific pain points in the modern developer workflow that Matis aims to solve.

**Responsibilities**:
- Document the current state of context loss.
- Identify the "Amalgamation Problem" (data silos).
- Highlight the cost of amnesiac AI tools.

## The Developer Context Gap
Current workflows look like this:

1. **Prompt**: You ask Claude for a solution.
2. **Execution**: You run commands, see errors, iterate.
3. **Forgotten**: The terminal buffer is cleared; the Claude session ends.
4. **New Tool**: You open Gemini for a different part of the task. It has zero awareness of the previous iteration.
5. **Git Commit**: You commit the "final" code. The reasoning is gone.

### Lost Artifacts
- **Lost Reasoning**: Why did we choose this library over that one?
- **Lost Experiments**: What failed before we found what worked?
- **Lost Failures**: Which errors did we see, and how did we fix them?
- **Lost Decisions**: Micro-decisions made during a flow state that never made it to a README.

## Pain Points
- **Context Switching**: Re-explaining the project to an AI tool every 20 minutes.
- **No Engineering Memory**: Relying on "feeling" and "vague memories" instead of hard data.
- **Documentation Rot**: Documentation is manual and trails reality. Matis aims to make "documentation" a side-effect of work.
- **Data Silos**: Your shell doesn't talk to your browser; your browser doesn't talk to your IDE; your AI agents talk to none of them.

## Future Work
- Quantitative analysis of time lost to context recovery.
- Mapping the "Context Decay" curve for typical engineering projects.
