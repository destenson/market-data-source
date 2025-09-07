#!/usr/bin/env python3
"""
Automated CHANGELOG generator for market-data-source project.
Generates changelog entries from git commits, version tags, and PR information.

Usage:
    python scripts/generate-changelog.py [--since=v0.3.0] [--output=CHANGELOG.md]
"""

import subprocess
import re
import sys
from datetime import datetime
from pathlib import Path
from typing import List, Dict, Tuple
import argparse

# Commit type mappings for conventional commits
COMMIT_TYPES = {
    'feat': 'Added',
    'fix': 'Fixed',
    'docs': 'Documentation',
    'style': 'Style',
    'refactor': 'Changed',
    'perf': 'Performance',
    'test': 'Tests',
    'build': 'Build',
    'ci': 'CI',
    'chore': 'Maintenance',
    'revert': 'Reverted',
    'break': 'BREAKING CHANGES'
}

# Keywords to categorize commits that don't follow conventional format
KEYWORD_CATEGORIES = {
    'Added': ['add', 'new', 'introduce', 'implement', 'create'],
    'Changed': ['update', 'modify', 'refactor', 'improve', 'enhance'],
    'Fixed': ['fix', 'resolve', 'correct', 'repair', 'patch'],
    'Removed': ['remove', 'delete', 'drop', 'clean'],
    'Security': ['security', 'vulnerability', 'cve'],
    'Deprecated': ['deprecate', 'obsolete'],
}

def run_git_command(cmd: List[str]) -> str:
    """Run a git command and return output."""
    try:
        result = subprocess.run(['git'] + cmd, capture_output=True, text=True, check=True)
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error running git command: {e}", file=sys.stderr)
        return ""

def get_tags() -> List[Tuple[str, str]]:
    """Get all version tags with dates."""
    tags_output = run_git_command(['tag', '-l', '--format=%(refname:short)|%(creatordate:short)'])
    if not tags_output:
        return []
    
    tags = []
    for line in tags_output.split('\n'):
        if '|' in line:
            tag, date = line.split('|')
            if re.match(r'^v?\d+\.\d+\.\d+', tag):
                tags.append((tag, date))
    
    return sorted(tags, reverse=True, key=lambda x: x[0])

def get_commits_between(from_tag: str = None, to_tag: str = None) -> List[Dict]:
    """Get commits between two tags."""
    if from_tag and to_tag:
        range_spec = f"{from_tag}..{to_tag}"
    elif from_tag:
        range_spec = f"{from_tag}..HEAD"
    else:
        range_spec = "HEAD"
    
    # Get commit info with a specific format
    log_format = "%H|%s|%b|%an|%ae|%ai"
    commits_output = run_git_command(['log', f'--format={log_format}', range_spec])
    
    if not commits_output:
        return []
    
    commits = []
    for line in commits_output.split('\n'):
        if '|' in line:
            parts = line.split('|')
            if len(parts) >= 6:
                commit = {
                    'hash': parts[0][:7],
                    'subject': parts[1],
                    'body': parts[2],
                    'author': parts[3],
                    'email': parts[4],
                    'date': parts[5]
                }
                commits.append(commit)
    
    return commits

def categorize_commit(commit: Dict) -> Tuple[str, str]:
    """Categorize a commit based on its message."""
    subject = commit['subject'].lower()
    
    # Check for conventional commit format
    match = re.match(r'^(\w+)(\([\w\-]+\))?:\s*(.+)', subject)
    if match:
        commit_type = match.group(1)
        if commit_type in COMMIT_TYPES:
            return COMMIT_TYPES[commit_type], commit['subject']
    
    # Check for keywords
    for category, keywords in KEYWORD_CATEGORIES.items():
        for keyword in keywords:
            if keyword in subject:
                return category, commit['subject']
    
    # Default to Changed
    return 'Changed', commit['subject']

def group_commits(commits: List[Dict]) -> Dict[str, List[str]]:
    """Group commits by category."""
    grouped = {}
    
    for commit in commits:
        category, message = categorize_commit(commit)
        
        # Skip certain commit types from changelog
        if category in ['Style', 'Tests', 'CI', 'Build', 'Maintenance']:
            continue
            
        if category not in grouped:
            grouped[category] = []
        
        # Clean up the message
        message = re.sub(r'^(feat|fix|docs|refactor|chore|test|perf|build|ci)(\([\w\-]+\))?:\s*', '', message, flags=re.IGNORECASE)
        message = message[0].upper() + message[1:] if message else message
        
        # Add commit hash for reference
        full_message = f"{message} ({commit['hash']})"
        grouped[category].append(full_message)
    
    return grouped

def generate_version_section(version: str, date: str, commits: List[Dict]) -> str:
    """Generate a changelog section for a version."""
    section = f"## [{version}] - {date}\n\n"
    
    grouped = group_commits(commits)
    
    # Order categories
    category_order = ['BREAKING CHANGES', 'Added', 'Changed', 'Deprecated', 'Removed', 'Fixed', 'Security', 'Performance', 'Documentation']
    
    for category in category_order:
        if category in grouped and grouped[category]:
            section += f"### {category}\n"
            for item in grouped[category]:
                section += f"- {item}\n"
            section += "\n"
    
    return section

def generate_changelog(since_tag: str = None, output_file: str = "CHANGELOG.md") -> None:
    """Generate the complete changelog."""
    output_path = Path(output_file)
    
    # Read existing changelog if it exists
    existing_content = ""
    if output_path.exists():
        with open(output_path, 'r') as f:
            existing_content = f.read()
    
    # Generate header
    header = """# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

"""
    
    # Get all tags
    tags = get_tags()
    
    # Generate new content
    new_content = header
    
    # Add unreleased section if there are commits since last tag
    if tags:
        unreleased_commits = get_commits_between(tags[0][0], None)
        if unreleased_commits:
            new_content += "## [Unreleased]\n\n"
            grouped = group_commits(unreleased_commits)
            
            category_order = ['BREAKING CHANGES', 'Added', 'Changed', 'Deprecated', 'Removed', 'Fixed', 'Security', 'Performance', 'Documentation']
            for category in category_order:
                if category in grouped and grouped[category]:
                    new_content += f"### {category}\n"
                    for item in grouped[category]:
                        new_content += f"- {item}\n"
                    new_content += "\n"
    
    # Process each version
    for i, (tag, date) in enumerate(tags):
        # Skip if we've reached the 'since' tag
        if since_tag and tag == since_tag:
            # Include existing content from this point if available
            if existing_content:
                marker = f"## [{tag}]"
                if marker in existing_content:
                    idx = existing_content.index(marker)
                    new_content += existing_content[idx:]
            break
        
        # Get commits for this version
        if i < len(tags) - 1:
            from_tag = tags[i + 1][0]
            commits = get_commits_between(from_tag, tag)
        else:
            # First version - get all commits up to this tag
            commits = get_commits_between(None, tag)
        
        if commits:
            version = tag.lstrip('v')
            new_content += generate_version_section(version, date, commits)
    
    # Add footer with links
    if tags:
        new_content += "\n---\n\n"
        new_content += "*This changelog was automatically generated. For the latest updates, see the [GitHub repository](https://github.com/yourusername/market-data-source).*\n\n"
        
        # Add comparison links
        for i, (tag, _) in enumerate(tags):
            version = tag.lstrip('v')
            if i < len(tags) - 1:
                from_tag = tags[i + 1][0]
                new_content += f"[{version}]: https://github.com/yourusername/market-data-source/compare/{from_tag}...{tag}\n"
            else:
                new_content += f"[{version}]: https://github.com/yourusername/market-data-source/releases/tag/{tag}\n"
    
    # Write the changelog
    with open(output_path, 'w') as f:
        f.write(new_content)
    
    print(f"✓ Changelog generated: {output_path}")
    print(f"  - Found {len(tags)} version tags")
    if tags:
        print(f"  - Latest version: {tags[0][0]}")

def main():
    parser = argparse.ArgumentParser(description='Generate CHANGELOG.md from git history')
    parser.add_argument('--since', help='Only regenerate from this version onwards (preserves older entries)')
    parser.add_argument('--output', default='CHANGELOG.md', help='Output file path')
    parser.add_argument('--dry-run', action='store_true', help='Print to stdout instead of writing file')
    
    args = parser.parse_args()
    
    if args.dry_run:
        # TODO: Implement dry-run mode
        print("Dry-run mode not yet implemented")
        sys.exit(1)
    
    generate_changelog(since_tag=args.since, output_file=args.output)

if __name__ == '__main__':
    main()